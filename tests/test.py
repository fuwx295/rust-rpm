import subprocess
import sys
import re
import statistics

def run_command_with_time(command):
    try:
        # Run the command with /usr/bin/time -v
        result = subprocess.run(['/usr/bin/time', '-v'] + command, stderr=subprocess.PIPE, text=True)
        
        # Extract Percent of CPU and maximum resident set size
        stderr_output = result.stderr
        percent_cpu = re.search(r'Percent of CPU this job got: (\d+)%', stderr_output)
        max_resident_set_size = re.search(r'Maximum resident set size \(kbytes\): (\d+)', stderr_output)
        
        if percent_cpu and max_resident_set_size:
            percent_cpu_value = int(percent_cpu.group(1))
            max_resident_set_size_value = int(max_resident_set_size.group(1))
            return percent_cpu_value, max_resident_set_size_value
        else:
            raise ValueError("Failed to parse time command output.")
    
    except Exception as e:
        print(f"An error occurred: {e}")
        return None

def main(command):
    percent_cpus = []
    max_resident_set_sizes = []

    for _ in range(100):
        result = run_command_with_time(command)
        if result:
            percent_cpu, max_resident_set_size = result
            percent_cpus.append(percent_cpu)
            max_resident_set_sizes.append(max_resident_set_size)
    
    if percent_cpus and max_resident_set_sizes:
        avg_percent_cpu = statistics.mean(percent_cpus)
        avg_max_resident_set_size = statistics.mean(max_resident_set_sizes)
        
        print(f"Average Percent of CPU this job got: {avg_percent_cpu:.2f}%")
        print(f"Average Maximum resident set size (kbytes): {avg_max_resident_set_size:.2f}")
    else:
        print("Failed to gather enough data to calculate averages.")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 script.py <command>")
        sys.exit(1)
    
    # Get the command from the command line arguments
    command
