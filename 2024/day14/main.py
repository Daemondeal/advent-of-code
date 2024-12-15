import sys

from PIL import Image
import numpy as np

from dataclasses import dataclass
from functools import reduce


@dataclass
class Robot:
    x: int
    y: int
    vx: int
    vy: int


def main():
    if len(sys.argv) != 2:
        print("USAGE: python main.py [input_file]")
        return

    with open(sys.argv[1], "r") as infile:
        input_content = infile.read()

    print(f"A: {solve_a(input_content)}")
    print(f"B: {solve_b(input_content)}")


def process_input(input: str) -> list[Robot]:
    robots = []

    for line in input.split("\n"):
        if line.strip() == "":
            continue

        p, v = line.split(" ")

        x, y = [int(xs) for xs in p.split("=")[1].split(",")]
        vx, vy = [int(xs) for xs in v.split("=")[1].split(",")]

        robot = Robot(x, y, vx, vy)
        robots.append(robot)

    return robots


# XLIMIT = 11
# YLIMIT = 7
XLIMIT = 101
YLIMIT = 103

# def detect_anomaly(robots):


def compute_framebuffer(robots):
    framebuffer = []

    for _ in range(XLIMIT):
        row = []
        for _ in range(YLIMIT):
            row.append(0)
        framebuffer.append(row)

    for robot in robots:
        framebuffer[robot.x][robot.y] += 1

    return framebuffer


def print_framebuffer(framebuffer):
    mat = np.hstack(framebuffer).reshape((XLIMIT, YLIMIT)) * 255
    image_arr = mat.clip(0, 255).astype("uint8").T
    image = Image.fromarray(image_arr)
    image.show()
    image.save(f"christmas_tree.png")


def simulate(robots, n):
    for _ in range(n):
        for robot in robots:
            robot.x += robot.vx
            robot.y += robot.vy
            robot.x %= XLIMIT
            robot.y %= YLIMIT


def get_quadrant(robot):
    if robot.x < (XLIMIT - 1) // 2:
        if robot.y < (YLIMIT - 1) // 2:
            return 0
        elif robot.y > (YLIMIT - 1) // 2:
            return 1
    elif robot.x > (XLIMIT - 1) // 2:
        if robot.y < (YLIMIT - 1) // 2:
            return 2
        elif robot.y > (YLIMIT - 1) // 2:
            return 3

    return 4


def simulate_step_with_fb(robots, framebuffer):
    for robot in robots:
        framebuffer[robot.x][robot.y] -= 1

        robot.x += robot.vx
        robot.y += robot.vy
        robot.x %= XLIMIT
        robot.y %= YLIMIT

        framebuffer[robot.x][robot.y] += 1


def solve_a(input) -> int:
    robots = process_input(input)
    simulate(robots, 100)

    quadrants = [0 for _ in range(4)]

    for robot in robots:
        quadrant = get_quadrant(robot)
        if quadrant < 4:
            quadrants[quadrant] += 1

    return reduce(lambda x, y: x * y, quadrants, 1)


def detect_square(framebuffer):
    fb = framebuffer
    for x in range(1, XLIMIT - 1):
        for y in range(1, YLIMIT - 1):

            square = fb[x][y] > 0

            if square:
                for i in range(-1, 2):
                    for j in range(-1, 2):
                        if fb[x + i][y + j] <= 0:
                            square = False
            if square:
                return True

    return False


def solve_b(inp) -> int:
    robots = process_input(inp)
    framebuffer = compute_framebuffer(robots)

    iteration = 0
    print()
    while True:
        simulate_step_with_fb(robots, framebuffer)
        iteration += 1

        print(f"Iteration: {iteration}...", end="\r")

        if detect_square(framebuffer):
            print_framebuffer(framebuffer)
            print(f"Iteration: {iteration}   ")
            should_stop = input("Accept Easter Egg? [y/N] ") == "y"

            if should_stop:
                return iteration


if __name__ == "__main__":
    main()
