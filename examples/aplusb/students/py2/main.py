def sum_values(val_x, val_y):
    total = val_x + val_y
    return total

def run_main():
    buffer = input().split()
    integer_a = int(buffer[0])
    integer_b = int(buffer[1])
    
    output_val = sum_values(integer_a, integer_b)
    print(output_val)

if __name__ == "__main__":
    run_main()
