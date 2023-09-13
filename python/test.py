
from lib import screen

from tqdm import tqdm
import time

def tq():
    prog = tqdm(range(5), desc="main...")
    for i in prog:
        prog.write("iter: {}".format(i))
        time.sleep(0.5)


def p():
    print(screen.CACHE_PATH)


if __name__ == "__main__":
    # tq()
    p()
