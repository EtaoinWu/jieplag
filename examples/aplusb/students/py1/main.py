def add_logic(alpha, beta):
    combined = alpha + beta
    return combined

def start_process():
    entry = input().split()
    first_num = int(entry[0])
    second_num = int(entry[1])
    
    final_result = add_logic(first_num, second_num)
    print(final_result)

if __name__ == "__main__":
    start_process()
