import re
import pandas as pd

server_log_pattern = r'(\S+) - - \[(.*?)\] "(\w+) (\S+?)(?=\s| HTTP) HTTP/\d\.\d" (\d+) (\d+) "-" "(.*?)"'
event_log_pattern = r'\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} [+-]\d{4})\] \[(\d+)\] \[(\w+)\] (.*)'

def parse_log_file(path):
    def process_log_line(line):
        server_log_match = re.match(server_log_pattern, line)
        event_log_match = re.match(event_log_pattern, line)

        if server_log_match:
            ip, timestamp, method, endpoint, response_code, _, user_agent = server_log_match.groups()
            log_data = {
                'Type': 'Server',
                'Timestamp': pd.to_datetime(timestamp, format='%d/%b/%Y:%H:%M:%S %z'),
                'IP': ip,
                'RequestMethod': method,
                'Endpoint': endpoint,
                'ResponseCode': response_code,
                'UserAgent': user_agent,
            }
            yield log_data

        elif event_log_match:
            timestamp, pid, level, message = event_log_match.groups()
            log_data = {
                'Type': 'Event',
                'Timestamp': pd.to_datetime(timestamp, format='%Y-%m-%d %H:%M:%S %z'),
                'PID': pid,
                'LogLevel': level,
                'EventMessage': message,
            }
            yield log_data

    logs = [log for line in open(path, 'r') for log in process_log_line(line)]

    server_log_df = pd.DataFrame([log for log in logs if log['Type'] == 'Server'])
    event_log_df = pd.DataFrame([log for log in logs if log['Type'] == 'Event'])

    return server_log_df, event_log_df
