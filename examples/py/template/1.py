# import numpy as np

# def test_example():
#     a = np.array([1, 2, 3])
#     b = np.array([1, 2, 3])
#     for i in range(3):
#         assert a[i] == b[i]
#     assert np.array_equal(a, b)

# def test_example2():
#     a = np.array([[1, 2, 3], [4, 5, 6]])
#     b = np.array([[1, 2, 3], [4, 5, 6]])
#     zero = np.zeros((2, 3))
#     for i in range(2):
#         for j in range(3):
#             assert a[i, j] == b[i, j]
#             assert zero[i, j] == 0
#     diff = a - b
#     assert np.array_equal(diff, zero)
    