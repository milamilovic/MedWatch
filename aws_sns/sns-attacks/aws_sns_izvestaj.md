# AWS SNS izveštaj

Analiziran je sistem za zdravstveni monitoring gde IoT uređaji šalju medicinske podatke pacijenata putem AWS SNS (Simple Notification Service) message brokera. Kao bezbednosna pretnja identifikovana je kategorija napada koji ciljaju integritet i dostupnost sistema prema CIA trijadi.

## Stablo napada
<img width="6634" height="1052" alt="mermaid-diagram-2026-02-18-014223" src="https://github.com/user-attachments/assets/8184164c-19f8-4136-9693-f013d6f33cca" />


### Praktično realizovan napad
Sistem se sastoji iz tri kontejnerizovane komponente: LocalStack koji je ustvari lokalna emulacija AWS SNS servisa, ranjive/mitigovane aplikacije implementirane u Rust-u i napadačkih skripti (dos_attack.py i message_injection.py) koje demonstriraju ranjivosti.

- #### Implementacija ranjive aplikacije
    Ranjiva aplikacija ima tri ranjivosti: 
        
        1. endpoint /send-sos prihvata proizvoljan sadržaj custom_message polja bez sanitizacije
        
        2. endpoint /send-bulk nema ograničenje broja poruka
        
        3. /admin/topics endpoint je dostupan svima bez autentifikacije. 
    
    Sve ove ranjivosti klasifikovane su kao CWE-20 (Improper Input Validation), CWE-284 (Improper Access Control), CWE-400 (Uncontrolled Resource Consumption) i CWE-770 (Allocation of Resources Without Limits).

- #### Implementacija napada
    Napad za message injection šalje HTTP POST zahteve na /send-sos endpoint sa proizvoljnim sadržajem u custom_message polju. Pošto sistem nema autentifikaciju, napadač bez ikakvih kredencijala može direktno da pozove API. Lažne poruke se šalju svim SMS pretplatnicima napadnute teme što može dovesti do nepotrebnih medicinskih intervencija ili zanemarivanja pravih upozorenja.

    DoS Message Flooding kreira 10 niti i svaka od njih šalje po 100 zahteva, odnosno ukupno se šalje 1000 zahteva. Svaki zahtev poziva /send-sos endpoint sa spam podacima. Pored toga, /send-bulk endpoint prihvata count parametar bez ograničenja, što omogućava slanje desetina hiljada poruka jednim zahtevom. Posledice napada su preopterećenje SNS servisa, iscrpljivanje AWS SMS kvota i troškova, gušenje pravih medicinskih upozorenja i smanjenje performansi servera. Napad spada u CWE-400 i CWE-770 prema MITRE terminologiji.

- #### Implementacija mitigovane aplikacije
    Mitigovana aplikacija dodaje JWT autentifikaciju, ograničenje veličine zahteva, validaciju opsega vrednosti, template za poruke, validaciju formata ID-ja pacijenta i dodaje logovanje. Na ovaj način pokretanje napada za denial of service rezultuje sa odgovorom 401 za svaki zahtev, a message injection više ne može da injektuje zbog toga što to polje nije deo API interfejsa aplikacije. 

- #### Video demonstracija
    
    https://github.com/user-attachments/assets/63b69a47-79df-4504-868d-93d60af80287
  
    https://github.com/user-attachments/assets/868f60b1-a432-4650-a0c2-d40a50a79579


### Napadi koji nisu realizovani
Preostala tri napada iz stabla nisu implementirana u praktičnom delu, ali su teorijski relevantni za potpunu bezbednosnu analizu sistema. Simple notification service je fully managed aws servis tako da ne postoje CVE koji su direktno vezani za sam servis ali su navedeni cwe i rizici u vezi loše konfiguracije.

- #### SSRF putem HTTP pretplate
    SNS može slati poruke na HTTP/HTTPS endpoint. Ako endpoint ne validira potpis poruke, napadač može simulirati SNS poruke. Ova ranjivost spada u CWE-918 (Server-Side Request Forgery) i CWE-345 (Insufficient Verification of Data Authenticity). Ukoliko aplikacija ne validira x-amz-sns-signature zaglavlje, napadač može poslati lažne notifikacije i ugroziti integritet sistema. Mitigacija za ovo je validacija SNS potpisa, verifikacija sertifikata i ograničavanje pristupa endpoint-u.

- #### AWS SDK ranjivost
    CVE-2020-28472
    
    U pojedinim verzijama AWS SDK-a za JavaScript postojala je ReDoS ranjivost (Regular expression Denial of Service) u AWS SDK. Ona može dovesti do iscrpljivanja CPU resursa zbog toga što ukoliko aplikacija koristi ranjivu verziju SDK-a za slanje SNS poruka, napadač može poslati specifično formatiran unos koji izaziva DoS. Mitigacija za ovu ranjivost je validacija unosa ili korišćenje 2.814.0 ili više verzije org.webjars.bower:aws-sdk biblioteke.

- #### Log4j ranjivost u subscriber aplikaciji
    CVE-2021-44228
    
    Ukoliko subscriber aplikacija koristi Log4j potencijalno je ranjiva na ovaj napad gde napadač može poslati malicioznu poruku preko SNS-a koja se loguje i aktivira RCE u aplikaciji. Ovo ne pogađa SNS servis direktno, ali pogađa sistem koji ga koristi i može dovesti do potpune nedostupnosti sistema.

- #### Data Exfiltration i Phishing
    Nepravilno konfigurisan IAM može omogućiti napadaču da dobije sns:Publish ili sns:DeleteTopic privilegije. Ovo spada u CWE-269 (Improper Privilege Management) i može da dovede do brisanja tema, slanja lažnih poruka i gašenja sistema za notifkacije. Mitigacija je pažljiva konfiguracija sistema.

## Reference
https://docs.aws.amazon.com/sns/latest/dg/sns-quotas.html

https://nvd.nist.gov/vuln/detail/CVE-2020-28472

https://security.snyk.io/vuln/SNYK-JAVA-ORGWEBJARSBOWER-1059426

https://nvd.nist.gov/vuln/detail/CVE-2021-44228

https://www.elastic.co/security-labs/aws-sns-abuse
