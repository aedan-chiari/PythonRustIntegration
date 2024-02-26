from rust_python_integration import (hello_world, exit_world,
                                     generate_random_one_d_array,
                                     calculate_total_sum,
                                     generate_increment_sum)
from numpy.random import randint
import numpy as np


if __name__ == "__main__":


    ## Hello World Called from Rust
    hello_string = hello_world()
    print(hello_string, "Called from Python", "\n")

    array1, array2 = generate_random_one_d_array()
    print("Array1 Dtype: ", type(array1))
    print("Array2 Dtype: ", type(array1), "\n")

    calculate_total_sum(array1, array2)
    print()

    increment_sum_array = generate_increment_sum(array1, array2)
    print(type(increment_sum_array), "\n")

    array1, array2 = (randint(0, 100, size=10, dtype=np.int32),
                      randint(0, 100, size=10, dtype=np.int32))
    print(array1, array2)
    print("Array1 Dtype: ", type(array1))
    print("Array2 Dtype: ", type(array2))

    # Demonstration of passing numpy generated arrays to a vec defined input/output function
    increment_sum_array2 = generate_increment_sum(array1, array2)
    print(type(increment_sum_array2), "\n")

    #Exit World Called from Rust
    exit_string = exit_world()
    print(exit_string, "Called from Python")