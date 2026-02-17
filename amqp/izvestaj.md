# AMQP izvestaj

Analiziran je sistem za zdravstveni monitoring gde IoT uređaji šalju medicinske podatke pacijenata putem AMQP message broker-a, konkretno RabbitMQ-a. Kao bezbednosna pretnja identifikovana je kategorija napda koji ciljaju dostupnost (availability) sistema prema CIA trijadi. 

## Stablo napada
<img width="4208" height="812" alt="image" src="https://github.com/user-attachments/assets/abca2d73-efee-44cb-8ac6-eb9b7e5b4399" />

### Praktično realizovan napad
Napad eksploatiše kombinaciju podrazumevanih kredencijala (guest/guest), odsustva ograničenja veličine poruka i odsustva ograničenja u broju redova. Sistem koji je implementiran se sastoji iz tri komponente koje su kontejnerizovane i organizovane pomoću docker compose-a.

- #### Implementacija ranjive aplikacije
    Ranjiv sistem se sastoji iz RabbbitMQ message brokera i aplikacije koja je implementirana u Rust programskom jeziku. RabbitMQ je pokrenut sa podrazumevanim kredencijalima za prijavu, i bez ograničenja u bilo kakvom korišćenju resursa. Aplikacija šalje simulirane podatke u json formatu bez validacija u broju ili dužini poruka.

- #### Implementacija napada
    Napad se izvodi pokretanjem aplikacije koja je implementirana u Rust-u koja izvodi Resource Exhaustion napad tako što prvo šalje 50 poruka od po 10MB, zatim kreira 1000 malicioznih redova i nakon toga šalje 10000 poruka čime izvodi message flooding. Slanjem velike količine podataka može da se aktivira OOM (out of memory) alarm koji blokira sve publisher-e, što se podrazumevano desi ako se zauzme 60 posto dostupne ram memorije. Svaki od 1000 kreiranih redova zauzima I/O resurse i prostor, a kreiraju se durable redovi koji postoje i nakon restarta brokera. Slanjem 10000 poruka se opterećuje obrada poruka, mešaju se maliciozne poruke sa pravim zdravstvenim podacima i usporava se neophodna komunikacija.

- #### Implementacija mitigovane aplikacije
    Mitigovan sistem se kao i ranjiv sastoji iz RabbitMQ message brokera i aplikacije u Rust-u, ali je RabbitMQ konfigurisan tako da su podrazumevani kredencijali za prijavu zamenjeni sa nalogom sa jakom šifrom i dodato je ograničenje i u broju poruka i u njihovoj dužini. Sama aplikacija za razliku od ranjive sadrži validaciju veličine poruka, rate limiting i ograničenjima u vezi queue-ova.

- #### Video demonstracija
    TODO

### Napadi koji nisu realizovani
Preostala četiri napada iz stabla nisu implementirani u praktičnom delu, ali su teorijski relevantni za potpunu bezbednosnu analizu sistema. 

- #### DoS HTTP API-ja
    Ukoliko napadač koji poseduje potrebne kredencijale počne da šalje izuzetno velike poruke putem HTTP Management API-ja (na portu 15672) može da dođe do iscrpljivanja resursa i broker može da padne. Razlika ovog napada u odnosu na implementirani je u tome što se ovde napad ne izvodi preko AMQP protokola, već preko HTTP-a i direktno je napad na RabbitMQ putem njegovog HTTP API-ja za slanje poruka. Pogođene verzije RabbitMQ brokera su < 3.11.24 i < 3.12.7. Mitigacije za ovaj napad su ograničenja prava pristupa Management API-ju i ograničavanje veličine poruka pomoću reverse proxy-ja ili upgrade na zakrpljenu verziju brokera.

- #### DoS consumer aplikacije
    Ranjivost se nalazi na klijentskoj strani u RabbitMQ biblioteci gde se ignoriše postavljeno ograničenje u dužini poruke što omogućava denial of service. Napadač koji ima potrebna prava pristupa može da pošalje poruku koja je veća od dostupne memorije procesa i time obori consumera koji dobije OOM error. Pogođene verzije biblioteke com.rabbitmq:amqp-client su < 5.14.3 / 5.16.1 / 5.17.1 / 5.18.0. Mitigacije za ovaj napad su ograničavanje veličine poruka na samom brokeru i unapređivanje na zakrpljenu verziju biblioteke.

- #### Loše kreiran AMQP
    Napadač bez ikakvih kredencijala može da šalje zlonamerne pakete koristeći AMQP sa verzijom ispod 1.0 (AMQP 0-8, 0-9, 0-91 and 0-10) i ti paketi su u stvari komande kreirane tako da obore broker-a. Mitigacije za ovo su validacija unosa i korišćenje broker-a koji je napravljen nad AMQP verzijom 1.0 ili više.

- #### Konfiguraciona manipulacija (policy injection)
    Napadač sa odgovarajućim pravima pristupa može da kreira destruktivne runtime policy-je tako da izazove prekid toga podataka bez obaranja brokera. Ovo može da se postigne postavljanjem TTL poruka na 0 čime poruke ističu čim pristignu u red pa consumer ne stigne da ih pročita. Drugi način je postavljanje maksimalne dužine poruka na 0 i overflow na reject-publish čime se sve nove poruke odbijaju. Mitigacije za ovaj napad su postavljanje prava pristupa tako da se publisher-ima dodele prava pisanja samo za jedan red bez configure prava. 

## Reference
https://www.rabbitmq.com/docs/memory

https://app.opencve.io/cve/CVE-2023-46118

https://app.opencve.io/cve/CVE-2023-46120

https://security.snyk.io/vuln/SNYK-JAVA-ORGAPACHEQPID-173747
