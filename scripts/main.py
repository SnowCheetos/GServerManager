import argparse
import datetime
import matplotlib.pyplot as plt
from data_processing import parse_log_file
from visualizer import visualize_logs

parser = argparse.ArgumentParser(description='Log Visualizer')
parser.add_argument('log_file', type=str, help='Path to the log file')
parser.add_argument('show', type=str, help='Whether or not to show the image')
args = parser.parse_args()

s_df, e_df = parse_log_file(args.log_file)
 
if len(s_df) > 0 and len(e_df) > 0:
    visualize_logs(s_df, e_df)
    plt.tight_layout()

    current_datetime = datetime.datetime.now()
    s_df.to_csv(f"data/logs/{args.log_file.split('/')[-1].split('.')[0]}-{current_datetime.strftime('%Y-%m-%d_%H-%M-%S')}-Server-Logs.csv")
    e_df.to_csv(f"data/logs/{args.log_file.split('/')[-1].split('.')[0]}-{current_datetime.strftime('%Y-%m-%d_%H-%M-%S')}-Event-Logs.csv")
    plt.savefig(f"data/figures/{args.log_file.split('/')[-1].split('.')[0]}-{current_datetime.strftime('%Y-%m-%d_%H-%M-%S')}.jpeg")
    
    if eval(args.show):
        plt.show()
    else:
        plt.close()

else:
    print(" Empty log file.")