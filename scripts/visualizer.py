import re
import argparse
import datetime
import pandas as pd
import matplotlib.pyplot as plt

log_entry_pattern = r'\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} [+-]\d{4})\] \[(\d+)\] \[(\w+)\] (.*)'
filter_pattern = r'Starting gunicorn|Shutting down: Master'

parser = argparse.ArgumentParser(description='Log Visualizer')
parser.add_argument('log_file', type=str, help='Path to the log file')
parser.add_argument('export', type=str, help='Whether or not to export the image')
args = parser.parse_args()

columns = ['Timestamp', 'PID', 'LogLevel', 'Message']

log_data = []
with open(args.log_file, 'r') as file:
    for line in file:
        match = re.match(log_entry_pattern, line)
        if match and re.search(filter_pattern, line):
            log_data.append(list(match.groups()))

df = pd.DataFrame(log_data, columns=columns)
df['Timestamp'] = pd.to_datetime(df['Timestamp'], format='%Y-%m-%d %H:%M:%S %z')
df[['RequestMethod', 'Endpoint', 'ResponseCode', 'UserAgent']] = df['Message'].str.extract(r'"(\w+) (.*?) HTTP/\d\.\d" (\d+) \d+ "-" "(.*?)"')

plt.figure(figsize=(10, 6))

plt.subplot(2, 2, 1)
df_start_stop = df[df['Message'].str.contains('Starting gunicorn|Shutting down: Master')]
plt.plot(df_start_stop['Timestamp'], df_start_stop['Message'])
plt.title('Server Start/Stop Events')
plt.xlabel('Timestamp')
plt.ylabel('Message')

plt.subplot(2, 2, 2)
response_code_counts = df['ResponseCode'].value_counts()
plt.bar(response_code_counts.index, response_code_counts.values)
plt.title('Response Code Distribution')
plt.xlabel('Response Code')
plt.ylabel('Count')

plt.subplot(2, 1, 2)
df['Hour'] = df['Timestamp'].dt.hour
request_counts_by_hour = df.groupby('Hour').size().reset_index(name='Count')
heatmap_data = pd.pivot_table(request_counts_by_hour, values='Count', index='Hour', columns='Hour')
plt.imshow(heatmap_data, cmap='hot', interpolation='nearest')
plt.colorbar(label='Request Count')
plt.title('Request Count by Hour')
plt.xlabel('Hour')
plt.ylabel('Hour')

plt.tight_layout()

if eval(args.export):
    current_datetime = datetime.datetime.now()
    plt.savefig(f"data/{args.log_file.split('/')[-1].split('.')[0]}-{current_datetime.strftime('%Y-%m-%d_%H-%M-%S')}.png")

plt.show()