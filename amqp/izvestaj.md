# AMQP izvestaj

Analiziran je sistem za zdravstveni monitoring gde IoT uređaji šalju medicinske podatke pacijenata putem AMQP message broker-a, konkretno RabbitMQ-a. Kao bezbednosna pretnja identifikovana je kategorija napda koji ciljaju dostupnost (availability) sistema prema CIA trijadi. 

## Stablo napada
<img width="4281" height="956" alt="image" src="https://github.com/user-attachments/assets/8e503eb8-a5e2-4287-8a1c-1b48d5742fa7" />


### Praktično realizovan napad
Napad eksploatiše kombinaciju podrazumevanih kredencijala (guest/guest), odsustva ograničenja veličine poruka i odsustva ograničenja u broju redova. Sistem koji je implementiran se sastoji iz tri komponente koje su kontejnerizovane i organizovane pomoću docker compose-a. Ovaj napad spada u CWE-400 i CWE-770 po mitre terminologiji, odnosno dešava se Uncontrolled Resource Consumption i Allocation of Resources Without Limits or Throttling.

- #### Implementacija ranjive aplikacije
    Ranjiv sistem se sastoji iz RabbbitMQ message brokera i aplikacije koja je implementirana u Rust programskom jeziku. RabbitMQ je pokrenut sa podrazumevanim kredencijalima za prijavu, i bez ograničenja u bilo kakvom korišćenju resursa. Aplikacija šalje simulirane podatke u json formatu bez validacija u broju ili dužini poruka.

- #### Implementacija napada
    Napad se izvodi pokretanjem aplikacije koja je implementirana u Rust-u koja izvodi Resource Exhaustion napad tako što prvo šalje 50 poruka od po 10MB, zatim kreira 1000 malicioznih redova i nakon toga šalje 10000 poruka čime izvodi message flooding. Slanjem velike količine podataka može da se aktivira OOM (out of memory) alarm koji blokira sve publisher-e, što se podrazumevano desi ako se zauzme 60 posto dostupne ram memorije. Svaki od 1000 kreiranih redova zauzima I/O resurse i prostor, a kreiraju se durable redovi koji postoje i nakon restarta brokera. Slanjem 10000 poruka se opterećuje obrada poruka, mešaju se maliciozne poruke sa pravim zdravstvenim podacima i usporava se neophodna komunikacija.

- #### Implementacija mitigovane aplikacije
    Mitigovan sistem se kao i ranjiv sastoji iz RabbitMQ message brokera i aplikacije u Rust-u, ali je RabbitMQ konfigurisan tako da su podrazumevani kredencijali za prijavu zamenjeni sa nalogom sa jakom šifrom i dodato je ograničenje i u broju poruka i u njihovoj dužini. Sama aplikacija za razliku od ranjive sadrži validaciju veličine poruka, rate limiting i ograničenjima u vezi queue-ova.

- #### Video demonstracija
    
https://github.com/user-attachments/assets/6a438d1e-c001-444f-89ef-e0c98fea016c


### Napadi koji nisu realizovani
Preostala četiri napada iz stabla nisu implementirani u praktičnom delu, ali su teorijski relevantni za potpunu bezbednosnu analizu sistema. 

- #### DoS HTTP API-ja
    CVE-2023-46118
  
    DoS ranjivost u RabbitMQ HTTP Management API-ju. Ukoliko napadač koji poseduje potrebne kredencijale počne da šalje izuzetno velike poruke putem HTTP Management API-ja (na portu 15672) može da dođe do iscrpljivanja resursa i broker može da padne zbog OOM greške koja obara proces brokera. Razlika ovog napada u odnosu na implementirani je u tome što se ovde napad ne izvodi preko AMQP protokola, već preko HTTP-a. Pogođene verzije RabbitMQ brokera su < 3.11.24 i < 3.12.6. Mitigacije za ovaj napad su ograničenja prava pristupa Management API-ju i ograničavanje veličine poruka pomoću reverse proxy-ja ili upgrade na zakrpljenu verziju brokera.

    Na primer endpoint PUT /api/exchanges/{vhost}/{name} kreira exchange i prihvata json u telu zahteva koji treba da izgleda ovako:
    ```
    {
      "type": "direct",
      "auto_delete": false,
      "durable": true,
      "internal": false,
      "arguments": {}
    }
    ```
    Zbog toga što u ranjivim verzijama API-ja ne postoji validacija veličine tog json tela zahteva, napadač može slati telo zahteva sa veoma velikim telom (mnogo većim od očekivanog). Zbog toga što svaki zahtev alocira memoriju na heap-u koja se ne oslobodi dok zahtev ne bude obrađen, dolazi do nagomilavanja memorijske potrošnje sve dok se RabbitMQ broker ne ugasi. Da bi se ovo izazvalo, dovoljan je HTTP klijent poput curl-a i automatizovana skripta koja neprestano šalje zahteve sa telom veličine nekoliko megabajta.

- #### DoS consumer aplikacije
    CVE-2023-46120
    
    Ranjivost se nalazi na klijentskoj strani u RabbitMQ biblioteci gde se ne poštuje postavljeno ograničenje u dužini poruke što omogućava denial of service. Napadač koji ima potrebna prava pristupa može da pošalje preveliku poruku i time obori consumera koji dobije OOM error. Pogođene verzije biblioteke com.rabbitmq:amqp-client su < 5.18.0. Mitigacije za ovaj napad su ograničavanje veličine poruka na samom brokeru i unapređivanje na zakrpljenu verziju biblioteke.

- #### Loše kreiran AMQP
    CVE-2021-22116
    
    Napadač bez ikakvih kredencijala može da šalje zlonamerne pakete koristeći AMQP sa verzijom ispod 1.0 i ti paketi su u stvari komande kreirane tako da obore broker-a. Mitigacije za ovo su validacija unosa i korišćenje RabbitMQ brokera sa verzijom >= 3.8.16.

- #### Brisanje redova
    CVE-2019-11287
  
    Ranjivost pogađa pogađa web management plugin RabbitMQ servera gde ne postoji validacija X-reason header-a u HTTP zaglavlju što omogućava napadaču da maliciozni Erlang format string koji se ekspanzuje tokom obrade i prekomerno troši memoriju na heap-u što izaziva pad servera i denial of service. Mitigacija za ovaj napad je unapređenje verzije RabbitMQ na > 3.8.1 ili > 3.7.21. 

## Reference
https://www.rabbitmq.com/docs/memory

https://app.opencve.io/cve/CVE-2023-46118

https://app.opencve.io/cve/CVE-2023-46120

https://security.snyk.io/vuln/SNYK-JAVA-ORGAPACHEQPID-173747

https://nvd.nist.gov/vuln/detail/CVE-2021-22116

https://nvd.nist.gov/vuln/detail/cve-2023-46120

https://nvd.nist.gov/vuln/detail/CVE-2019-11287

https://www.rabbitmq.com/docs/http-api-reference
