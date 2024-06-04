import time
from tqdm import tqdm
import random

def delay():
  time.sleep(random.uniform(0.02, 0.06))
  sum_value = np.random.randint(1, 42)
  for i in range(420):
    sv1 = np.random.randint(1, 42)
    sv2 = np.random.randint(1, 42)
    sum_value += sv1 * sv2

def test_tqdm():
  for i in tqdm(range(42)):
    delay()
