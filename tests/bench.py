import subprocess
import sys

def run_command(command):
    result = subprocess.run(f'/usr/bin/time -p {" ".join(command)}', stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True, universal_newlines=True)
    return result.stderr

def parse_time_output(output):
    user_time = 0.0
    sys_time = 0.0
    real_time = 0.0
    for line in output.splitlines():
        if line.startswith("user"):
            user_time += float(line.split()[1])
        elif line.startswith("sys"):
            sys_time += float(line.split()[1])
        elif line.startswith("real"):
            real_time += float(line.split()[1])
    return user_time, sys_time, real_time

def main(command):
    total_user_time = 0.0
    total_sys_time = 0.0
    total_real_time = 0.0
    iterations = 1000

    for _ in range(iterations):
        output = run_command(command)
        user_time, sys_time, real_time = parse_time_output(output)
        total_user_time += user_time
        total_sys_time += sys_time
        total_real_time += real_time

    avg_user_time = total_user_time / iterations
    avg_sys_time = total_sys_time / iterations
    avg_real_time = total_real_time / iterations

    print(f"Average user time: {avg_user_time:.4f} seconds")
    print(f"Average sys time: {avg_sys_time:.4f} seconds")
    print(f"Average real time: {avg_real_time:.4f} seconds")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 bench.py <command>")
        sys.exit(1)
    
    command = sys.argv[1:]
    main(command)
