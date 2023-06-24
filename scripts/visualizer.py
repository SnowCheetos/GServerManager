import matplotlib.pyplot as plt
import seaborn as sns

def visualize_logs(server_log_df, event_log_df):
    fig = plt.figure(figsize=(12, 9))

    # HTTP Method Distribution
    ax1 = fig.add_subplot(221)
    method_counts = server_log_df['RequestMethod'].value_counts()
    sns.barplot(x=method_counts.index, y=method_counts.values, alpha=0.8, ax=ax1)
    ax1.set_title('HTTP Method Distribution')
    ax1.set_ylabel('Number of Occurrences', fontsize=12)
    ax1.set_xlabel('Method', fontsize=12)

    # Status Code Distribution
    ax2 = fig.add_subplot(222)
    status_counts = server_log_df['ResponseCode'].value_counts()
    sns.barplot(x=status_counts.index, y=status_counts.values, alpha=0.8, ax=ax2)
    ax2.set_title('Status Code Distribution')
    ax2.set_ylabel('Number of Occurrences', fontsize=12)
    ax2.set_xlabel('Status Code', fontsize=12)

    # Requests Over Time
    ax3 = fig.add_subplot(223)
    server_log_df['Timestamp'] = server_log_df['Timestamp'].dt.floor('T')  # rounding time to the nearest minute
    requests_time = server_log_df['Timestamp'].value_counts().sort_index()
    sns.lineplot(x=requests_time.index, y=requests_time.values, ax=ax3)
    ax3.set_title('Requests Over Time')
    ax3.set_ylabel('Number of Requests', fontsize=12)
    ax3.set_xlabel('Time', fontsize=12)

    # Event Type Distribution
    ax4 = fig.add_subplot(224)
    event_counts = event_log_df['LogLevel'].value_counts()
    sns.barplot(x=event_counts.index, y=event_counts.values, alpha=0.8, ax=ax4)
    ax4.set_title('Event Type Distribution')
    ax4.set_ylabel('Number of Occurrences', fontsize=12)
    ax4.set_xlabel('Event Type', fontsize=12)