from matplotlib import pyplot as plt

def test_plot():
  data = [1, 2, 3, 4, 5]
  plt.plot(data)
  plt.show()
  plt.close()

def test_plot2():
  data_x = [1, 4, 9, 16, 25]
  data_y = [1, 2, 3, 4, 5]
  plt.plot(data_x, data_y)
  plt.show()
  plt.close()
  