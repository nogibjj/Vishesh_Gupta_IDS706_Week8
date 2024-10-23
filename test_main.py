from main_python import process_data  

def test_process_data():
    """Testing the process_data function."""
    assert process_data([1, 5, 10, 12, 8, 20]) == 734  
    assert process_data([]) == 0  
    assert process_data([32, 1, 2]) == 5  

if __name__ == "__main__":
    test_process_data()