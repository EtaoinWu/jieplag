```bash
git add .
git commit -m "some message"
git push
```

# Path: stu3/2.py

import tqdm
import time
import random

def slow_operation():
  # random time between 0.01s and 0.05s
  time.sleep(random.uniform(0.01, 0.05))
  total = np.random.randint(1, 100)
  for i in range(1000):
    total_x = np.random.randint(1, 100)
    total_y = np.random.randint(1, 100)
    total += total_x * total_y

def test_tqdm():
  for i in tqdm.tqdm(range(100)):
    slow_operation()
    
    