import subprocess
import time

def run_command(command):
    start = time.time()
    result = subprocess.run(command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    end = time.time()
    real_time = end - start
    return real_time

def average_times(command, num_runs=100):
    total_real = 0

    for _ in range(num_runs):
        real = run_command(command)
        total_real += real

    avg_real = total_real / num_runs

    return avg_real

command = "./rust-rpm -qb -a"
avg_real= average_times(command)

print(f"Average Real Time: {avg_real:.3f} seconds")

