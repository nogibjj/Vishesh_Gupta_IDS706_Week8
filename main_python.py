import time
import psutil

def process_data(data):

    # We are squaring each number and filtering for values less than 100 and obtaining the sum of those numbers

    squared_data = [x ** 2 for x in data]
    filtered_data = [x for x in squared_data if x <= 1000]
    total_sum = sum(filtered_data)
    
    return total_sum

def get_memory_usage():
    process = psutil.Process()
    mem_info = process.memory_info()
    return mem_info.rss / 1024 ** 2  # Convert bytes to MB

# Measure the running time and memory usage
def measure_performance(data):

    initial_memory = get_memory_usage()
    start_time = time.time()
    result = process_data(data)
    end_time = time.time()
    final_memory = get_memory_usage()
    
    # Calculate time and memory used
    runtime = end_time - start_time
    memory_used = final_memory - initial_memory
    
    print(f"Processed Result: {result}")
    print(f"Running Time: {runtime:.6f} seconds")
    print(f"Memory Usage: {memory_used:.6f} MB")

# Example usage
data = [i for i in range(1, 999)]
measure_performance(data)
