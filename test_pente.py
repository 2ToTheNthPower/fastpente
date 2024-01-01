from pente import Pente
import numpy as np
import time
import pente

NUM_GAMES = 100000

# states = []

start = time.time()
for i in range(NUM_GAMES):
    done = False
    game = Pente()

    states = game.play_random_game()

    current_game = states[len(states) - 2][0]
    current_player = current_game.current_player

    values = pente.get_values(game=current_game, rollouts_per_action=75)
    # print(np.array(states[min([100, len(states) - 3])][0].get_board()).sum())
    values = np.array(values)

    probs = (values + 1) / (2) + 0.01 * np.random.uniform(low=-1, high=1, size=values.shape)
    print(probs)
    # print(len(states))

end = time.time()

print(f"Time per game: {(end - start) / NUM_GAMES} seconds")
print(f"Total time: {end - start} seconds")