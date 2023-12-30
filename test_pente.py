from pente import Pente
import numpy as np
import time

NUM_GAMES = 100000

states = []

start = time.time()
for i in range(NUM_GAMES):
    done = False
    game = Pente()

    states += game.play_random_game()
    print(len(states))

end = time.time()

print(f"Time per game: {(end - start) / NUM_GAMES} seconds")
print(f"Total time: {end - start} seconds")